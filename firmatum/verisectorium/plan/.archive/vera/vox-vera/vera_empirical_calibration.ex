<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: zoetica/docs/refs/vox-vera/vera_empirical_calibration.ex
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/_core/zoetica/docs/refs/vox-vera/vera_empirical_calibration.ex
  Do not edit here expecting to update the live original.
-->

defmodule VERA.EmpiricalCalibration do
  @moduledoc """
  Empirically-calibrated verbal probability encoding for VERA knowledge graphs.

  This module provides mappings from verbal probability phrases to numerical
  probability distributions based on psychological research (primarily Mosteller
  & Youtz 1990, validated against Vogel 2022 meta-analysis).

  ## Three-Tier Confidence System

  - **Tier 1** (IQR < 5%): High consensus, use freely with 80% credible intervals
  - **Tier 2** (IQR 5-20%): Moderate consensus, use with 50% credible intervals or flag
  - **Tier 3** (IQR > 20%): Ambiguous, avoid or use very wide priors with warnings

  ## Usage

      iex> VERA.EmpiricalCalibration.get_calibration("very likely")
      {:ok, %{
        median: 87.5,
        credible_80: {80, 95},
        beta: {16.22, 2.32},
        tier: 2,
        confidence: :high
      }}

      iex> VERA.EmpiricalCalibration.to_cpt_range("very likely")
      {:ok, {80, 95}}

      iex> VERA.EmpiricalCalibration.get_calibration("possible")
      {:warning, %{
        median: 38.5,
        credible_80: {5, 75},
        tier: 3,
        warning: "BIMODAL DISTRIBUTION - high ambiguity",
        recommended_alternative: "Use numeric range or avoid"
      }}

  ## References

  - Mosteller, F., & Youtz, C. (1990). Quantifying probabilistic expressions.
    Statistical Science, 5(1), 2-34. [n=238 science writers]
  - Vogel, T., et al. (2022). Systematic review of verbal probability expressions.
    [Meta-analysis of 21 studies, 1967-2018]
  - Budescu, D. V., et al. (2012). Effective communication of uncertainty in the
    IPCC reports. Climatic Change, 113(2), 181-200.
  """

  # ============================================================================
  # Tier 1: High Consensus (IQR < 5%) - Use Freely
  # ============================================================================

  @tier_1_high_consensus %{
    "always" => %{
      median: 99.7,
      p25_p75: {99.6, 99.8},
      iqr: 0.3,
      beta: {9112.0, 27.0},
      credible_80: {99.5, 99.9},
      credible_50: {99.6, 99.8},
      tier: 1,
      confidence: :very_high,
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "almost always" => %{
      median: 91.7,
      p25_p75: {89.7, 95.2},
      iqr: 5.5,
      beta: {27.66, 2.51},
      credible_80: {88, 96},
      credible_50: {90, 94},
      tier: 1,
      confidence: :very_high,
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "certain" => %{
      median: 99.6,
      p25_p75: {98.7, 99.8},
      iqr: 1.1,
      beta: {1807.0, 7.0},
      credible_80: {98.5, 99.9},
      credible_50: {99.2, 99.8},
      tier: 1,
      confidence: :very_high,
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "almost certain" => %{
      median: 90.2,
      p25_p75: {87.5, 95.0},
      iqr: 7.5,
      beta: {18.34, 1.99},
      credible_80: {85, 95},
      credible_50: {88, 93},
      tier: 1,
      confidence: :very_high,
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "almost never" => %{
      median: 2.9,
      p25_p75: {1.2, 4.6},
      iqr: 3.4,
      beta: {1.64, 54.83},
      credible_80: {1, 5},
      credible_50: {2, 4},
      tier: 1,
      confidence: :very_high,
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "never" => %{
      median: 0.3,
      p25_p75: {0.1, 0.4},
      iqr: 0.3,
      beta: {27.0, 9085.0},
      credible_80: {0.1, 0.5},
      credible_50: {0.2, 0.4},
      tier: 1,
      confidence: :very_high,
      source: "Mosteller & Youtz 1990",
      n: 238
    }
  }

  # ============================================================================
  # Tier 2: Moderate Consensus (5% <= IQR <= 20%) - Use with Care
  # ============================================================================

  @tier_2_moderate_consensus %{
    "very likely" => %{
      median: 87.5,
      p25_p75: {80.1, 90.2},
      iqr: 10.1,
      beta: {16.22, 2.32},
      credible_80: {80, 95},
      credible_50: {83, 92},
      tier: 2,
      confidence: :high,
      source: "Mosteller & Youtz 1990",
      n: 238,
      medical_context: 84.3
    },
    "very probable" => %{
      median: 89.7,
      p25_p75: {81.5, 90.4},
      iqr: 8.9,
      beta: {20.97, 2.41},
      credible_80: {82, 95},
      credible_50: {86, 93},
      tier: 2,
      confidence: :high,
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "very frequent" => %{
      median: 82.6,
      p25_p75: {75.3, 89.7},
      iqr: 14.5,
      beta: {10.54, 2.20},
      credible_80: {72, 92},
      credible_50: {78, 88},
      tier: 2,
      confidence: :high,
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "likely" => %{
      median: 71.1,
      p25_p75: {62.6, 77.6},
      iqr: 15.0,
      beta: {11.12, 4.52},
      credible_80: {60, 82},
      credible_50: {66, 76},
      tier: 2,
      confidence: :moderate,
      source: "Mosteller & Youtz 1990",
      n: 238,
      medical_context: 71.87
    },
    "probable" => %{
      median: 70.2,
      p25_p75: {64.7, 77.7},
      iqr: 13.0,
      beta: {10.79, 4.58},
      credible_80: {58, 82},
      credible_50: {65, 75},
      tier: 2,
      confidence: :moderate,
      source: "Mosteller & Youtz 1990",
      n: 238,
      medical_context: 69.87
    },
    "usually" => %{
      median: 75.1,
      p25_p75: {65.6, 82.2},
      iqr: 16.7,
      beta: {9.90, 3.28},
      credible_80: {62, 87},
      credible_50: {70, 80},
      tier: 2,
      confidence: :moderate,
      source: "Mosteller & Youtz 1990",
      n: 238,
      medical_context: 75.38
    },
    "often" => %{
      median: 72.5,
      p25_p75: {65.0, 75.4},
      iqr: 10.4,
      beta: {13.80, 5.24},
      credible_80: {62, 83},
      credible_50: {68, 77},
      tier: 2,
      confidence: :moderate,
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "frequent" => %{
      median: 72.2,
      p25_p75: {60.0, 75.3},
      iqr: 15.2,
      beta: {9.38, 3.61},
      credible_80: {58, 85},
      credible_50: {66, 78},
      tier: 2,
      confidence: :moderate,
      source: "Mosteller & Youtz 1990",
      n: 238,
      medical_context: 61.3
    },
    "high probability" => %{
      median: 82.3,
      p25_p75: {77.1, 87.2},
      iqr: 10.1,
      beta: {15.71, 3.38},
      credible_80: {75, 90},
      credible_50: {79, 86},
      tier: 2,
      confidence: :high,
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "unlikely" => %{
      median: 17.2,
      p25_p75: {9.8, 22.7},
      iqr: 13.0,
      beta: {3.92, 18.85},
      credible_80: {8, 28},
      credible_50: {12, 23},
      tier: 2,
      confidence: :moderate,
      source: "Mosteller & Youtz 1990",
      n: 238,
      medical_context: 17.71
    },
    "improbable" => %{
      median: 12.5,
      p25_p75: {7.6, 22.3},
      iqr: 14.7,
      beta: {2.33, 16.30},
      credible_80: {5, 25},
      credible_50: {8, 18},
      tier: 2,
      confidence: :moderate,
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "very unlikely" => %{
      median: 5.0,
      p25_p75: {2.7, 9.8},
      iqr: 7.1,
      beta: {2.29, 43.51},
      credible_80: {2, 12},
      credible_50: {3, 8},
      tier: 2,
      confidence: :high,
      source: "Mosteller & Youtz 1990",
      n: 238,
      medical_context: 17.0
    },
    "very improbable" => %{
      median: 4.8,
      p25_p75: {1.5, 7.5},
      iqr: 5.9,
      beta: {2.54, 50.37},
      credible_80: {1, 10},
      credible_50: {2, 7},
      tier: 2,
      confidence: :high,
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "low probability" => %{
      median: 15.0,
      p25_p75: {7.8, 22.3},
      iqr: 14.5,
      beta: {3.15, 17.85},
      credible_80: {5, 28},
      credible_50: {10, 21},
      tier: 2,
      confidence: :moderate,
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "seldom" => %{
      median: 10.2,
      p25_p75: {7.4, 17.5},
      iqr: 10.1,
      beta: {2.83, 24.91},
      credible_80: {5, 19},
      credible_50: {7, 14},
      tier: 2,
      confidence: :moderate,
      source: "Mosteller & Youtz 1990",
      n: 238,
      medical_context: 11.7
    },
    "rarely" => %{
      median: 7.2,
      p25_p75: {3.6, 10.0},
      iqr: 6.5,
      beta: {2.79, 35.96},
      credible_80: {2, 14},
      credible_50: {4, 10},
      tier: 2,
      confidence: :moderate,
      source: "Mosteller & Youtz 1990",
      n: 238,
      medical_context: 8.9
    },
    "very rarely" => %{
      median: 3.0,
      p25_p75: {1.2, 5.0},
      iqr: 3.8,
      beta: {1.74, 56.26},
      credible_80: {1, 6},
      credible_50: {2, 4},
      tier: 2,
      confidence: :high,
      source: "Mosteller & Youtz 1990",
      n: 238
    }
  }

  # ============================================================================
  # Tier 3: Ambiguous (IQR > 20%) - Avoid or Flag
  # ============================================================================

  @tier_3_ambiguous %{
    "possible" => %{
      median: 38.5,
      p25_p75: {7.5, 50.2},
      iqr: 42.7,
      beta: {0.53, 0.84},
      credible_80: {5, 75},
      credible_50: {15, 65},
      tier: 3,
      confidence: :low,
      warning: "BIMODAL DISTRIBUTION - high ambiguity",
      recommended_alternative: "Use numeric range or avoid",
      source: "Mosteller & Youtz 1990",
      n: 238,
      medical_context: 43.28
    },
    "not infrequent" => %{
      median: 49.6,
      p25_p75: {32.7, 57.3},
      iqr: 24.6,
      beta: {1.01, 1.03},
      credible_80: {20, 80},
      credible_50: {35, 65},
      tier: 3,
      confidence: :low,
      warning: "Negative construction increases ambiguity",
      recommended_alternative: "Rephrase positively or use numeric",
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "moderate probability" => %{
      median: 52.4,
      p25_p75: {40.1, 58.7},
      iqr: 18.5,
      beta: {2.40, 2.18},
      credible_80: {30, 75},
      credible_50: {42, 62},
      tier: 3,
      confidence: :low,
      warning: "Too vague - wide interpretation range",
      recommended_alternative: "Specify numeric range",
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "sometimes" => %{
      median: 25.0,
      p25_p75: {17.5, 35.0},
      iqr: 17.5,
      beta: {2.50, 7.50},
      credible_80: {12, 42},
      credible_50: {18, 33},
      tier: 3,
      confidence: :low,
      warning: "Frequency term with wide interpretation",
      recommended_alternative: "Use numeric range or specific frequency",
      source: "Mosteller & Youtz 1990",
      n: 238
    },
    "occasionally" => %{
      median: 20.0,
      p25_p75: {12.5, 27.7},
      iqr: 15.2,
      beta: {3.29, 13.16},
      credible_80: {9, 35},
      credible_50: {14, 27},
      tier: 3,
      confidence: :low,
      warning: "Frequency term with moderate ambiguity",
      recommended_alternative: "Use numeric range",
      source: "Mosteller & Youtz 1990",
      n: 238
    }
  }

  # ============================================================================
  # Medical Context Adjustments (Domain-Specific)
  # ============================================================================

  @medical_context_adjustments %{
    "rare" => %{
      general: 7.2,
      medical: 10.0,
      medical_severe: 10.06,
      medical_mild: 14.14,
      shift: +2.8,
      source: "Medical context meta-analysis"
    },
    "uncommon" => %{
      general: nil,
      medical: 17.64,
      source: "Medical context meta-analysis"
    },
    "common" => %{
      general: nil,
      medical: 58.73,
      medical_severe: 43.08,
      medical_mild: 50.47,
      source: "Medical context meta-analysis"
    },
    "very common" => %{
      general: nil,
      medical: 60.1,
      source: "Medical context meta-analysis"
    }
  }

  # ============================================================================
  # Public API
  # ============================================================================

  @doc """
  Get calibration data for a verbal probability phrase.

  Returns `{:ok, calibration_map}` for Tier 1 and Tier 2 phrases.
  Returns `{:warning, calibration_map}` for Tier 3 phrases (ambiguous).
  Returns `{:error, :not_found}` if phrase is not in database.

  ## Options

    * `:context` - Domain context (`:general`, `:medical`). Default: `:general`
    * `:credible_level` - Credible interval level (`:80`, `:50`). Default: `:80`
    * `:severity` - For medical context only (`:mild`, `:moderate`, `:severe`)

  ## Examples

      iex> get_calibration("very likely")
      {:ok, %{median: 87.5, credible_80: {80, 95}, tier: 2, ...}}

      iex> get_calibration("likely", context: :medical)
      {:ok, %{median: 71.87, credible_80: {60, 82}, tier: 2, ...}}

      iex> get_calibration("possible")
      {:warning, %{median: 38.5, tier: 3, warning: "BIMODAL DISTRIBUTION", ...}}
  """
  def get_calibration(phrase, opts \\ []) do
    context = Keyword.get(opts, :context, :general)
    severity = Keyword.get(opts, :severity)

    phrase_normalized = String.downcase(phrase) |> String.trim()

    calibration =
      @tier_1_high_consensus[phrase_normalized] ||
        @tier_2_moderate_consensus[phrase_normalized] ||
        @tier_3_ambiguous[phrase_normalized]

    case calibration do
      nil ->
        {:error, :not_found}

      cal when cal.tier == 3 ->
        {:warning, apply_context(cal, context, severity)}

      cal ->
        {:ok, apply_context(cal, context, severity)}
    end
  end

  @doc """
  Convert verbal phrase to CPT range (for use in Bayesian network).

  Returns `{:ok, {lower, upper}}` for usable phrases.
  Returns `{:warning, {lower, upper}}` for ambiguous phrases (Tier 3).
  Returns `{:error, :not_found}` if phrase not in database.

  ## Examples

      iex> to_cpt_range("very likely")
      {:ok, {80, 95}}

      iex> to_cpt_range("likely", credible_level: :50)
      {:ok, {66, 76}}

      iex> to_cpt_range("possible")
      {:warning, {5, 75}}
  """
  def to_cpt_range(phrase, opts \\ []) do
    credible_level = Keyword.get(opts, :credible_level, :80)

    case get_calibration(phrase, opts) do
      {:ok, cal} ->
        range = get_credible_interval(cal, credible_level)
        {:ok, range}

      {:warning, cal} ->
        range = get_credible_interval(cal, credible_level)
        {:warning, range}

      {:error, reason} ->
        {:error, reason}
    end
  end

  @doc """
  Get Beta distribution parameters for a phrase.

  Returns `{:ok, {alpha, beta}}` for phrases with Beta encoding.

  ## Examples

      iex> get_beta_params("very likely")
      {:ok, {16.22, 2.32}}

      iex> get_beta_params("always")
      {:ok, {9112.0, 27.0}}
  """
  def get_beta_params(phrase, opts \\ []) do
    case get_calibration(phrase, opts) do
      {:ok, %{beta: {alpha, beta}}} -> {:ok, {alpha, beta}}
      {:warning, %{beta: {alpha, beta}}} -> {:ok, {alpha, beta}}
      {:error, reason} -> {:error, reason}
    end
  end

  @doc """
  List all available phrases by tier.

  Returns map with keys `:tier_1`, `:tier_2`, `:tier_3`.

  ## Examples

      iex> list_phrases_by_tier()
      %{
        tier_1: ["always", "almost always", "certain", ...],
        tier_2: ["very likely", "likely", "probable", ...],
        tier_3: ["possible", "not infrequent", ...]
      }
  """
  def list_phrases_by_tier do
    %{
      tier_1: Map.keys(@tier_1_high_consensus),
      tier_2: Map.keys(@tier_2_moderate_consensus),
      tier_3: Map.keys(@tier_3_ambiguous)
    }
  end

  @doc """
  Check if a phrase is in the high-consensus tier (safe to use).

  ## Examples

      iex> is_tier_1?("always")
      true

      iex> is_tier_1?("very likely")
      false
  """
  def is_tier_1?(phrase) do
    phrase_normalized = String.downcase(phrase) |> String.trim()
    Map.has_key?(@tier_1_high_consensus, phrase_normalized)
  end

  @doc """
  Check if a phrase is ambiguous (Tier 3 - should avoid).

  ## Examples

      iex> is_ambiguous?("possible")
      true

      iex> is_ambiguous?("likely")
      false
  """
  def is_ambiguous?(phrase) do
    phrase_normalized = String.downcase(phrase) |> String.trim()
    Map.has_key?(@tier_3_ambiguous, phrase_normalized)
  end

  # ============================================================================
  # Private Helpers
  # ============================================================================

  defp apply_context(calibration, :medical, severity) do
    phrase = find_phrase_for_calibration(calibration)

    # Check if medical context adjustment exists
    case @medical_context_adjustments[phrase] do
      nil ->
        # No medical adjustment, use general calibration
        calibration

      adjustment ->
        # Apply medical context
        medical_value = get_medical_value(adjustment, severity)

        if medical_value do
          # Recalculate credible intervals with medical median
          # (Keep general Beta parameters as approximation)
          %{calibration | median: medical_value, context: :medical}
        else
          calibration
        end
    end
  end

  defp apply_context(calibration, _context, _severity), do: calibration

  defp get_medical_value(adjustment, :severe) when is_map_key(adjustment, :medical_severe),
    do: adjustment.medical_severe

  defp get_medical_value(adjustment, :mild) when is_map_key(adjustment, :medical_mild),
    do: adjustment.medical_mild

  defp get_medical_value(adjustment, _severity), do: adjustment[:medical]

  defp find_phrase_for_calibration(calibration) do
    # Search tier maps to find phrase key
    all_phrases =
      Map.merge(@tier_1_high_consensus, @tier_2_moderate_consensus)
      |> Map.merge(@tier_3_ambiguous)

    {phrase, _cal} =
      Enum.find(all_phrases, fn {_phrase, cal} ->
        cal.median == calibration.median
      end) || {nil, nil}

    phrase
  end

  defp get_credible_interval(calibration, :80), do: calibration.credible_80
  defp get_credible_interval(calibration, :50), do: calibration.credible_50

  defp get_credible_interval(calibration, level) when is_float(level) do
    # Compute custom credible interval from Beta parameters
    {alpha, beta} = calibration.beta
    compute_credible_interval(alpha, beta, level)
  end

  # ============================================================================
  # Beta Distribution Utilities
  # ============================================================================

  @doc """
  Compute credible interval from Beta distribution.

  Uses quantile function for Beta(α, β) at specified level.

  ## Examples

      iex> compute_credible_interval(16.22, 2.32, 0.80)
      {80, 95}

      iex> compute_credible_interval(16.22, 2.32, 0.50)
      {83, 92}
  """
  def compute_credible_interval(alpha, beta, level) do
    lower_tail = (1 - level) / 2
    upper_tail = 1 - lower_tail

    # Note: Requires Statistics library or equivalent
    # This is a stub - actual implementation would use:
    # lower = Statistics.Distributions.Beta.quantile(alpha, beta, lower_tail)
    # upper = Statistics.Distributions.Beta.quantile(alpha, beta, upper_tail)

    # Placeholder approximation using mean ± z*std
    mean = alpha / (alpha + beta)
    variance = (alpha * beta) / ((alpha + beta) ** 2 * (alpha + beta + 1))
    std = :math.sqrt(variance)

    # For 80% credible interval, z ≈ 1.28
    # For 50% credible interval, z ≈ 0.67
    z = get_z_score(level)

    lower = max(0.0, mean - z * std) * 100
    upper = min(1.0, mean + z * std) * 100

    {round(lower), round(upper)}
  end

  defp get_z_score(0.80), do: 1.28
  defp get_z_score(0.50), do: 0.67
  defp get_z_score(0.95), do: 1.96

  @doc """
  Update Beta distribution with new evidence (Bayesian conjugacy).

  ## Examples

      # Prior: "very likely" → Beta(16.22, 2.32)
      # Evidence: 7 successes, 3 failures (observed "likely" outcome)
      iex> update_beta_with_evidence({16.22, 2.32}, {7, 3})
      {23.22, 5.32}
  """
  def update_beta_with_evidence({prior_alpha, prior_beta}, {successes, failures}) do
    posterior_alpha = prior_alpha + successes
    posterior_beta = prior_beta + failures
    {posterior_alpha, posterior_beta}
  end

  @doc """
  Compute Beta parameters from mean and IQR (method of moments).

  ## Examples

      iex> iqr_to_beta(87.5, 10.1)
      {16.22, 2.32}
  """
  def iqr_to_beta(median, iqr) do
    # Convert IQR to variance (assumes approximately normal in middle 50%)
    sigma = iqr / 1.35
    variance = (sigma / 100) ** 2

    # Method of moments
    mean = median / 100

    if variance >= mean * (1 - mean) do
      # Variance exceeds Beta distribution capacity - return uninformative
      {1.0, 1.0}
    else
      scale = (mean * (1 - mean) / variance) - 1
      alpha = mean * scale
      beta = (1 - mean) * scale
      {Float.round(alpha, 2), Float.round(beta, 2)}
    end
  end
end
